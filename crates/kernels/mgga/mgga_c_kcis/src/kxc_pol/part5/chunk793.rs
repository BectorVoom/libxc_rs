//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 793/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk793<F: Float>(t174: F, t6887: F, t638: F, t1881: F, t1886: F, t2133: F, t6284: F, t447: F, t637: F, t446: F, t1885: F, t2132: F, t1650: F, t2011: F, t4171: F, t4170: F, t4160: F, sigma2: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t175 = t174 <= zeta_threshold;
    let t6888 = sigma2 * t6887;
    let t6889 = t6888 * t638;
    let t6890 = t6889 / 8.0;
    let t6891 = t1881 * t1886;
    let t6892 = t6891 / 8.0;
    let t6893 = t1881 * t2133;
    let t6894 = t6893 / 8.0;
    let t6895 = piecewise3(t175, 0.0, t6284);
    let t6896 = t447 * t6895;
    let t6897 = t6896 * t637;
    let t6898 = t446 * t6897;
    let t6899 = t6898 / 16.0;
    let t6900 = t1885 * t2132;
    let t6901 = t446 * t6900;
    let t6902 = t6901 / 8.0;
    let t6903 = t1650 * t2011;
    let t6904 = t4171 * t6903;
    let t6905 = t4170 * t6904;
    let t6906 = t4160 * t6905;
    (t6888, t6890, t6892, t6894, t6896, t6899, t6902, t6905, t6906)
}

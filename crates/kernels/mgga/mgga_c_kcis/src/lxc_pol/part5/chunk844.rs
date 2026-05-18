//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 844/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk844<F: Float>(t174: F, t236: F, t6883: F, t233: F, t1926: F, t638: F, t1881: F, t1886: F, t2133: F, t6284: F, t447: F, t637: F, t446: F, sigma2: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t175 = t174 <= zeta_threshold;
    let t6884 = t236 * t6883;
    let t6885 = t233 * t6884;
    let t6886 = t6885 / F::new(16.0);
    let t6887 = F::new(1.0) / t1926;
    let t6888 = sigma2 * t6887;
    let t6889 = t6888 * t638;
    let t6890 = t6889 / F::new(8.0);
    let t6891 = t1881 * t1886;
    let t6892 = t6891 / F::new(8.0);
    let t6893 = t1881 * t2133;
    let t6894 = t6893 / F::new(8.0);
    let t6895 = piecewise3::<f64>(t175, F::new(0.0), t6284);
    let t6896 = t447 * t6895;
    let t6897 = t6896 * t637;
    let t6898 = t446 * t6897;
    (t6884, t6886, t6888, t6890, t6892, t6894, t6896, t6898)
}

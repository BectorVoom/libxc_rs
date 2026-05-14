//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1201/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1201<F: Float>(t1597: F, t8335: F, t4375: F, t1586: F, t394: F, t8306: F, t20: F, t2734: F, t2738: F, t27958: F) -> (F, F, F, F, F, F, F, F) {
    let t34934 = t1597 * t8335;
    let t34935 = t4375 * t34934;
    let t34936 = t1586 * t34935;
    let t34939 = t8306 * t394;
    let t34940 = t34939 * t20;
    let t34941 = t2734 * t34940;
    let t34944 = t2738 * t27958;
    let t34945 = t1586 * t34944;
    (t34934, t34935, t34936, t34939, t34940, t34941, t34944, t34945)
}

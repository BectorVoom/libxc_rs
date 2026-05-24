//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 570/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk570<F: Float>(t1248: F, t4065: F, t7736: F, t1249: F, t7740: F, t7744: F, t4049: F, t4061: F, t6020: F, t6066: F, t7914: F, t7917: F, t7920: F, t7932: F, t7939: F, t7945: F, t7947: F) -> (F, F, F, F) {
    let t7951 = t1248 * t4065 * t7736;
    let t7954 = t1248 * t1249 * t7740;
    let t7957 = t1248 * t1249 * t7744;
    let t7959 = -F::new(0.9494625e0) * t7932 + F::new(0.1898925e1) * t7939 + t4049 + F::cast_from(0.19931111111111111111e0_f64) * t6020 - F::cast_from(0.19931111111111111111e0_f64) * t7914 + F::cast_from(0.59793333333333333334e0_f64) * t7917 - F::cast_from(0.29896666666666666667e0_f64) * t7920 + F::new(0.15358125e0) * t7945 + F::new(0.3071625e0) * t7947 + t4061 + F::cast_from(0.21908444444444444444e0_f64) * t6066 - F::cast_from(0.5477111111111111111e-1_f64) * t7951 + F::cast_from(0.32862666666666666666e0_f64) * t7954 - F::cast_from(0.16431333333333333333e0_f64) * t7957;
    (t7951, t7954, t7957, t7959)
}

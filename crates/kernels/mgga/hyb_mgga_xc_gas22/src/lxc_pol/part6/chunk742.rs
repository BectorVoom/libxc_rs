//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 742/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk742<F: Float>(t3827: F, t69: F, t3844: F, t608: F, t1941: F, t612: F, t3847: F, t3849: F, t3851: F, t3853: F, t3855: F, t3857: F, t3859: F, t3861: F, t3863: F, t3865: F, t51: F, t565: F) -> (F, F, F, F, F) {
    let t3867 = t69 * t3827;
    let t3869 = t608 * t3844;
    let t3871 = t1941 * t3827;
    let t3873 = t612 * t3844;
    let t3875 = t51 * t3827 / F::new(6.0) - t565 * t3844 / F::new(18.0) - t3847 / F::new(48.0) + t3849 / F::new(240.0) + t3851 / F::new(640.0) - t3853 / F::new(4480.0) - t3855 / F::new(11520.0) + t3857 / F::new(103680.0) + t3859 / F::new(258048.0) - t3861 / F::new(2838528.0) - t3863 / F::new(6881280.0) + t3865 / F::cast_from(89456640.0_f64) + t3867 / F::new(0.21233664e9) - t3869 / F::new(0.31850496e10) - t3871 / F::new(0.74317824e10) + t3873 / F::cast_from(0.1263403008e12_f64);
    (t3867, t3869, t3871, t3873, t3875)
}

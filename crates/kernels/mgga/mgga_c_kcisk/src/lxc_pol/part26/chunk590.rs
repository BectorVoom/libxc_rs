//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 590/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk590<F: Float>(t5821: F, t7: F, t171: F, t1398: F, t5601: F, t156: F, t3122: F, t1375: F, t2198: F, t960: F, t2201: F, t965: F, t220: F, t3844: F, t3848: F, t3851: F, t3853: F, t3857: F, t3860: F, t5802: F, t5804: F, t5808: F, t5816: F, t5817: F) -> (F, F, F, F, F, F, F, F) {
    let t5822 = t7 * t5821;
    let t5823 = t171 * t5822;
    let t5824 = t1398 * t5601;
    let t5827 = t156 * t3122;
    let t5828 = t1375 * t5601;
    let t5831 = t960 * t2198;
    let t5833 = t965 * t2201;
    let t5835 = -0.11955719325063177623e-1 * t5802 + 0.10359077815592613752e-3 * t5804 + 0.23911438650126355246e-1 * t3857 * t220 - 0.10359077815592613752e-3 * t5808 * t220 - 0.11955719325063177623e-1 * t3853 + 0.10359077815592613752e-3 * t3860 - t3844 - t3848 + t3851 + 0.1585e-2 * t5816 * t5817 + 0.10082625e-4 * t5823 * t5824 - 0.7026e-2 * t5827 * t5828 + 0.4684e-2 * t5831 - 0.13208333333333333333e-2 * t5833;
    (t5822, t5823, t5824, t5827, t5828, t5831, t5833, t5835)
}

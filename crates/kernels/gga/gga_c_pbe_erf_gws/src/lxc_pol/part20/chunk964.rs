//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 964/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk964<F: Float>(t10874: F, t2741: F, t2753: F, t3563: F, t582: F, t616: F, t10863: F, t10866: F, t10870: F, t10873: F, t5384: F, t5387: F, t5417: F, t7715: F, t7728: F, t7732: F, t7734: F, t7736: F, t7753: F, t7757: F) -> (F, F, F, F) {
    let t10875 = F::new(8.0) / F::new(45.0) * t10874;
    let t10876 = t2741 * t2753;
    let t10877 = F::new(16.0) / F::new(45.0) * t10876;
    let t10878 = t582 * t3563;
    let t10879 = t616 * t10878;
    let t10880 = F::new(8.0) / F::new(45.0) * t10879;
    let t10882 = t10863 + t10866 - t7715 - t5384 + t5387 + t7728 + t7732 + F::new(2.0) / F::new(3.0) * t7734 + F::new(0.2431111111111111111e0) * t7736 - t10870 - t10873 - t10875 - t7753 + t7757 + t10877 + t10880 + t5417 / F::new(3.0);
    (t10875, t10877, t10880, t10882)
}

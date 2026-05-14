//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1093/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1093<F: Float>(t2512: F, t2485: F, t2492: F, t23605: F, t23608: F, t23612: F, t23614: F, t23616: F, t23653: F, t23655: F, t23670: F, t23673: F, t23676: F, t23679: F, t23682: F, t23620: F, t23622: F, t23624: F, t23626: F, t23630: F, t23633: F, t23635: F, t23637: F, t23640: F, t23644: F, t23660: F) -> (F, F, F, F) {
    let t24752 = t2512 * t2512;
    let t24759 = t2485 * t2492;
    let t24775 = -0.41095999999999999999e0 * t23605 + 0.41095999999999999998e0 * t23670 - 0.34246666666666666665e-1 * t23608 - 0.4566222222222222222e-1 * t23673 - 0.11415555555555555555e0 * t23676 + 0.41096e0 * t23612 - 0.61644e0 * t23679 + 0.9132444444444444444e-1 * t23614 + 0.13698666666666666667e0 * t23616 - 0.13698666666666666667e0 * t23653 + 0.4566222222222222222e-1 * t23655;
    let t24776 = 0.17757530864197530864e0 * t23682;
    let t24788 = t24776 - 0.45662222222222222221e-1 * t23620 - 0.3044148148148148148e-1 * t23622 + 0.22831111111111111111e-1 * t23624 + 0.25367901234567901233e-1 * t23626 - 0.50735802469135802467e-1 * t23630 - 0.17123333333333333333e-1 * t23633 + 0.71030123456790123454e-1 * t23635 - 0.9132444444444444444e-1 * t23637 + 0.2283111111111111111e0 * t23640 + 0.10274e0 * t23644 + 0.13698666666666666667e0 * t23660;
    (t24752, t24759, t24775, t24788)
}

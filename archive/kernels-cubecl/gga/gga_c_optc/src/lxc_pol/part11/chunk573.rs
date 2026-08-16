//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 573/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk573<F: Float>(t4623: F, t5: F, t2024: F, t675: F, t2126: F, t4630: F, t127: F, t4649: F, t116: F, t2010: F, t4599: F, t4595: F, t627: F) -> (F, F, F, F, F, F) {
    let t4679 = t5 * t4623;
    let t4680 = t4679 * t2024;
    let t4681 = t675 * t4680;
    let t4685 = t2126 * t4630;
    let t4689 = t5 * t4649 * t127;
    let t4690 = t675 * t4689;
    let t4693 = t4679 * t127;
    let t4694 = t675 * t4693;
    let t4699 = t2010 * t116 * t4599;
    let t4703 = t627 * t116 * t4595;
    (t4681, t4685, t4690, t4694, t4699, t4703)
}

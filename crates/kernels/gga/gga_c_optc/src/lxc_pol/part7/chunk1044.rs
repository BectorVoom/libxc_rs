//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1044/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1044<F: Float>(t6742: F, t6751: F, t1796: F, t509: F, t6636: F, t6739: F, t6642: F, t1772: F, t1998: F, t6748: F, t1994: F, t6814: F) -> (F, F, F, F, F, F, F, F) {
    let t22699 = t6742 * t6751;
    let t22700 = F::new(0.1926377843805564792e1) * t22699;
    let t22703 = F::new(0.38024868119570572865e2) * t1796 * t509 * t6636;
    let t22704 = t6742 * t6739;
    let t22705 = F::new(0.65061485296689145286e-1) * t22704;
    let t22708 = F::new(0.21687161765563048428e-1) * t1796 * t509 * t6642;
    let t22711 = F::new(0.43374323531126096856e-1) * t1796 * t1772 * t1998;
    let t22712 = t6742 * t6748;
    let t22713 = F::new(0.86748647062252193714e-1) * t22712;
    let t22716 = F::new(0.1284251895870376528e1) * t1796 * t1772 * t1994;
    let t22719 = F::new(0.38527556876111295841e1) * t1796 * t509 * t6814;
    (t22700, t22703, t22705, t22708, t22711, t22713, t22716, t22719)
}

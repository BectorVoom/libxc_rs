//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1098/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1098<F: Float>(t1100: F, t13442: F, t213: F, t2440: F, t2405: F, t27659: F, t13521: F, t27595: F, t24324: F, t24330: F, t27515: F, t33432: F, t3789: F, t5585: F, t108495: F, t108561: F, t108739: F, t13395: F, t13519: F, t13520: F, t13522: F, t17819: F, t231: F, t2413: F, t24270: F, t2428: F, t25057: F, t27527: F, t27642: F, t27651: F, t27652: F, t27658: F, t27669: F, t27673: F, t27695: F, t27712: F, t3759: F, t6023: F, t6027: F, t6034: F, t6035: F, t65688: F, t66680: F, t6798: F, t6819: F, t96612: F, t96623: F) -> (F, F, F) {
    let t108897 = t1100 * t13442;
    let t108920 = t2440 * t213;
    let t108922 = t27659 * t108920 * t2405;
    let t108925 = t27595 * t13521;
    let t108940 = t24324 * t24330 * t27515;
    let t108943 = t3789 * t33432 * t5585;
    let t108949 = -0.88910709717637694816e-2 * t108897 * t25057 * t27712 * t2428 - 0.29693535778629056444e-3 * t96612 + 0.25876656037945937584e-6 * t65688 * t27669 * t27673 - 0.17024962234567901235e-1 * t96623 - 0.12768721675925925926e-1 * t27651 * t6035 * t27652 * t2413 + 0.10338048737805743098e-3 * t66680 * t6798 * t13522 + 0.11877414311451622578e-3 * t6034 * t27642 * t24270 + 0.46509801892875584e-2 * t3759 * t27695 * t13395 - 0.20182686335885480796e-3 * t27658 * t108922 - 0.51690243689028715488e-4 * t13520 * t6023 * t108925 - 0.25845121844514357744e-4 * t13520 * t6023 * t108495 + 0.12020514968855939808e-5 * t17819 * t13519 * t6027 * t108739 - 0.10338048737805743097e-4 * t27527 * t6023 * t108561 - 0.76612330055555555555e-1 * t108940 - 0.27246626553445399075e-2 * t108943 * t6819 * t231 * t213 * t2428;
    (t108922, t108925, t108949)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1307/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1307<F: Float>(t12411: F, t23809: F, t105064: F, t554: F, t93178: F, t23724: F, t6604: F, t1014: F, t104671: F, t104735: F, t104813: F, t104838: F, t104851: F, t105066: F, t105102: F, t105110: F, t105117: F, t105167: F, t1355: F, t2001: F, t2036: F, t2043: F, t23728: F, t23810: F, t26601: F, t26617: F, t39852: F, t40087: F, t48841: F, t8852: F, t93169: F, t94434: F, t94700: F, t94761: F, t94876: F) -> (F, F) {
    let t105260 = t12411 * t23809;
    let t105270 = t93178 * t105064 * t554;
    let t105279 = t23724 * t6604;
    let t105293 = 0.22226000364197530866e-1 * t94876 - 0.93056218143801431977e1 * t8852 * t104838 + 0.43791161479435967988e1 * t105260 * t26617 + 0.11300578175490223804e0 * t94700 * t105117 - 0.81472461409953017303e-1 * t2001 * t105167 - 0.43791161479435967988e1 * t40087 * t105066 + 0.43791161479435967988e1 * t39852 * t105270 + 0.13335600218518518519e0 * t94434 * t93169 * t104735 * t104671 - 0.10947790369858991997e1 * t48841 * t26601 - 0.28251445438725559511e-1 * t94761 * t105279 - 0.46528109071900715989e1 * t2036 * t23728 * t1014 - 0.76518236253115177207e1 * t2043 * t104851 + 0.10947790369858991997e1 * t8852 * t105110 + 0.22653425206514361674e0 * t1355 * t104813 + 0.18611243628760286396e2 * t23810 * t105102;
    (t105270, t105293)
}

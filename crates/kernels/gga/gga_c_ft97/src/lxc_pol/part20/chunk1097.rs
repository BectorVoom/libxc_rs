//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1097/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1097<F: Float>(t108880: F, t24389: F, t108845: F, t108848: F, t108857: F, t108860: F, t108871: F, t108874: F, t13407: F, t13572: F, t13589: F, t2405: F, t2428: F, t25057: F, t27651: F, t27658: F, t27669: F, t27670: F, t27673: F, t27695: F, t27700: F, t27723: F, t3723: F, t3759: F, t52588: F, t6034: F, t6035: F, t6036: F, t65684: F, t704: F, t9543: F, t96451: F, t96540: F, t96559: F, t96586: F, t992: F) -> (F,) {
    let t108881 = t108880 * t24389;
    let t108893 = -0.17263005832038132093e-5 * t96540 + 0.42562405586419753086e-2 * t96559 + 0.27039520901431665706e-3 * t3723 * t108845 - 0.13519760450715832853e-3 * t9543 * t108848 - 0.38306165027777777778e-1 * t96451 * t6035 * t704 * t992 * t2428 + 0.85124811172839506173e-2 * t108857 - 0.10357803499222879255e-4 * t27670 * t108860 - 0.51789017496114396277e-5 * t65684 * t27669 * t27673 + 0.22270151833971792333e-3 * t6034 * t6035 * t6036 * t13572 - 0.15137014751914110597e-3 * t27658 * t108871 - 0.17024962234567901235e-1 * t27651 * t6035 * t108874 * t2405 + 0.12768721675925925926e-1 * t96586 + 0.13519760450715832853e-3 * t9543 * t108881 - 0.23254900946437792e-1 * t3759 * t27695 * t13407 - 0.81118562704294997117e-4 * t13589 * t27700 + 0.13336606457645654222e-1 * t52588 * t25057 * t27723 * t2428;
    (t108893,)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1150/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1150<F: Float>(t8155: F, t9333: F, t1572: F, t16251: F, t3354: F, t14626: F, t597: F, t10370: F, t4614: F, t574: F, t2859: F, t30949: F, t10447: F, t1562: F, t10324: F, t1641: F) -> (F, F, F, F, F, F, F) {
    let t34153 = 0.21450293971110256002e1 * t8155 * t9333;
    let t34156 = 0.15889106645266856297e0 * t1572 * t16251 * t3354;
    let t34178 = 0.51123901271894332903e1 * t597 * t14626 * t3354;
    let t34181 = 0.12269736305254639897e2 * t574 * t4614 * t10370;
    let t34186 = 0.14300195980740170668e1 * t2859 * t30949;
    let t34189 = 0.18404604457881959845e2 * t1562 * t4614 * t10447;
    let t34191 = 0.12269736305254639897e2 * t1641 * t10324;
    (t34153, t34156, t34178, t34181, t34186, t34189, t34191)
}

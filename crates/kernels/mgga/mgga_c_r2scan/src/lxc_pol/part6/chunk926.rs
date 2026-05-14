//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 926/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk926<F: Float>(t2185: F, t277: F, t495: F, t360: F, t2133: F, t2139: F, t2236: F, t2254: F, t2582: F, t549: F, t6501: F, t6505: F, t6509: F, t6513: F, t6515: F, t6519: F, t6522: F, t6524: F, t6528: F, t6530: F, t6538: F, t6543: F, t6545: F, t6568: F, t6572: F, t6576: F, t6580: F, t6583: F) -> (F, F, F, F) {
    let t6584 = t277 * t2185;
    let t6585 = t6584 * t495;
    let t6586 = t360 * t6585;
    let t6589 = -0.41607464352260489103e1 * t6501 - 0.38140175656238781678e1 * t6505 - 0.12713391885412927226e1 * t6509 - 0.12805040077930161442e1 * t6513 + 0.38415120233790484326e0 * t6515 - 0.11524536070137145298e1 * t6519 - 0.65854491829355115988e0 * t6522 * t6524 - 0.2600466522016280569e1 * t6528 * t6530 + 0.34930954652346593433e-1 * t6538 - 0.17465477326173296717e-1 * t6543 - 0.14636160809074174528e-1 * t6545 - 0.13002332610081402845e0 * t2236 * t2254 - 0.43341108700271342816e-1 * t549 * t6568 + 0.13002332610081402845e0 * t2133 * t6572 + 0.39006997830244208535e0 * t2139 * t6576 - 0.13002332610081402845e0 * t2582 * t6580 - 0.26004665220162805689e0 * t6583 * t6586;
    (t6584, t6585, t6586, t6589)
}

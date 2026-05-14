//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 886/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk886<F: Float>(t6263: F, t783: F, t785: F, t1610: F, t1616: F, t1234: F, t133: F, t1605: F, t1604: F, t1579: F, t2133: F, t2139: F, t2184: F, t574: F, t6196: F, t6200: F, t6205: F, t6215: F, t6218: F, t6221: F, t6225: F, t6228: F, t6232: F, t6236: F, t6241: F, t6246: F, t6250: F, t6254: F, t6260: F) -> (F, F, F, F, F) {
    let t6266 = 0.73613752582167450608e0 * t783 * t785 * t6263;
    let t6268 = t783 * t1610 * t1616;
    let t6271 = t1605 * t133 * t1234;
    let t6272 = t1604 * t6271;
    let t6274 = -0.43371823197556470519e-3 * t6196 + 0.26004665220162805689e0 * t2184 * t6200 + 0.26004665220162805689e0 * t6205 * t1579 - 0.19043987679069580388e-1 * t6215 - 0.7801399566048841707e0 * t6218 * t6221 - 0.43341108700271342816e-1 * t574 * t6225 - 0.38415120233790484326e0 * t6228 - 0.69345773920434148506e0 * t6232 - 0.20803732176130244552e1 * t6236 - 0.64025200389650807209e0 * t6241 - 0.49390868872016336989e-1 * t6246 + 0.39006997830244208535e0 * t2139 * t6250 + 0.13002332610081402845e0 * t2133 * t6254 - t6260 + t6266 - 0.2037639021386884617e0 * t6268 + 0.16463622957338778996e-1 * t6272;
    (t6266, t6268, t6271, t6272, t6274)
}

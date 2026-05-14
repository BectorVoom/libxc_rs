//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1357/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1357<F: Float>(t20818: F, t20820: F, t2604: F, t2139: F, t2294: F, t7978: F, t2183: F, t7983: F, t20422: F, t20424: F, t8123: F, t20434: F, t8129: F, t20862: F, t20865: F, t20869: F, t20874: F, t20886: F, t20894: F, t20896: F, t6425: F, t6586: F, t8220: F) -> (F,) {
    let t25715 = t20818 * t2604 * t20820;
    let t25718 = t2139 * t2294 * t7978;
    let t25720 = t2183 * t7983;
    let t25726 = t20422 * t8123 * t20424;
    let t25728 = t20434 * t8129;
    let t25729 = 0.19043987679069580388e-1 * t25728;
    let t25733 = -0.38087975358139160777e-1 * t20862 - 0.11426392607441748233e0 * t20865 + 0.55488507004364032914e1 * t20869 - 0.34930954652346593433e-1 * t20874 + 0.86743646395112941037e-3 * t25715 - 0.20803732176130244552e1 * t25718 - 0.26004665220162805689e0 * t25720 * t6586 + 0.39006997830244208535e0 * t6425 * t8220 - 0.25705033881751801528e-4 * t25726 - t25729 + 0.38415120233790484326e0 * t20886 - 0.43371823197556470519e-3 * t20894 + 0.34672886960217074253e0 * t20896;
    (t25733,)
}

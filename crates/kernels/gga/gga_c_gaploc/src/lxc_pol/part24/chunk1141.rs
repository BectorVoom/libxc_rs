//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1141/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1141<F: Float>(t2610: F, t7291: F, t20019: F, t8775: F, t10978: F, t5771: F, t20671: F, t24501: F, t28309: F, t10847: F, t22706: F, t7584: F, t16455: F, t32889: F, t7585: F, t10948: F, t33067: F, t33069: F, t33072: F, t33074: F, t33077: F, t33079: F, t33080: F, t33081: F, t33084: F, t7736: F) -> (F,) {
    let t33087 = t2610 * t7291;
    let t33090 = 0.55611873258433997041e0 * t8775 * t20019 * t33087;
    let t33092 = 0.14300195980740170668e1 * t5771 * t10978;
    let t33094 = t28309 * t20671 * t24501;
    let t33095 = 0.17041300423964777634e0 * t33094;
    let t33098 = 0.30674340763136599742e2 * t7584 * t22706 * t10847;
    let t33101 = 0.23005755572352449806e2 * t16455 * t7585 * t32889;
    let t33102 = -t33067 + t33069 + t33072 + t33074 - t33077 - t33079 - t33080 + t33081 - t33084 - 0.10725146985555128001e1 * t10948 * t7736 + t33090 + t33092 + t33095 - t33098 + t33101;
    (t33102,)
}

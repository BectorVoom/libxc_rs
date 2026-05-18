//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1282/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1282<F: Float>(t28357: F, t28361: F, t11025: F, t2087: F, t4614: F, t2610: F, t7291: F, t20019: F, t8775: F, t10978: F, t5771: F, t20671: F, t24501: F, t28309: F) -> (F, F, F, F, F, F) {
    let t33080 = F::new(0.63904876589867916128e-1) * t28357;
    let t33081 = F::new(0.15976219147466979032e0) * t28361;
    let t33084 = F::new(0.18404604457881959845e2) * t2087 * t4614 * t11025;
    let t33087 = t2610 * t7291;
    let t33090 = F::new(0.55611873258433997041e0) * t8775 * t20019 * t33087;
    let t33092 = F::new(0.14300195980740170668e1) * t5771 * t10978;
    let t33094 = t28309 * t20671 * t24501;
    (t33080, t33081, t33084, t33090, t33092, t33094)
}

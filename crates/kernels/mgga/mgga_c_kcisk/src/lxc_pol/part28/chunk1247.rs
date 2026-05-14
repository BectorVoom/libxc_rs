//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1247/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1247<F: Float>(t1775: F, t35382: F, t8510: F, t9741: F, t5006: F, t8518: F, t2642: F, t2647: F, t33197: F, t7261: F) -> (F, F, F, F, F, F, F, F) {
    let t35383 = t1775 * t35382;
    let t35388 = t9741 * t8510;
    let t35389 = t5006 * t35388;
    let t35394 = t9741 * t8518;
    let t35395 = t1775 * t35394;
    let t35400 = t2647 * t2642;
    let t35401 = t33197 * t35400;
    let t35402 = t7261 * t35401;
    (t35383, t35388, t35389, t35394, t35395, t35400, t35401, t35402)
}

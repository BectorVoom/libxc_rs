//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1096/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1096<F: Float>(t3351: F, t890: F, t10818: F, t27763: F, t1113: F, t2832: F, t775: F, t2430: F, t11061: F, t33: F, t2408: F, t10489: F, t1940: F, t1963: F, t2000: F, t2403: F, t25436: F, t25440: F, t25445: F, t25752: F, t25767: F, t25778: F, t25781: F, t27158: F, t4541: F, t7087: F, t7091: F, t7200: F, t7207: F, t92742: F, t92775: F, t92822: F, t93404: F) -> (F,) {
    let t94276 = t3351 * t890;
    let t94280 = t27763 * t10818;
    let t94286 = t1113 * t2832;
    let t94293 = t3351 * t775;
    let t94297 = t1113 * t2430;
    let t94312 = t33 * t11061;
    let t94316 = t1113 * t2408;
    let t94320 = t33 * t10489;
    let t94324 = 9.0 * t4541 * t7087 * t25752 - 3.0 / 2.0 * t1940 * t7091 * t94276 + 9.0 * t27158 * t94280 - 3.0 / 2.0 * t1940 * t92775 * t7207 - 3.0 / 2.0 * t1940 * t7091 * t94286 + 3.0 * t1940 * t93404 * t25778 + 9.0 / 2.0 * t2403 * t1963 * t94293 + 9.0 / 2.0 * t2403 * t1963 * t94297 + 9.0 / 2.0 * t2403 * t25436 * t7200 + 9.0 / 2.0 * t2403 * t7087 * t25767 - 3.0 * t1940 * t25440 * t25781 + 3.0 * t92822 * t2000 - 3.0 * t1940 * t92742 * t94312 + 3.0 * t1940 * t25445 * t94316 + 3.0 / 2.0 * t2403 * t1963 * t94320;
    (t94324,)
}

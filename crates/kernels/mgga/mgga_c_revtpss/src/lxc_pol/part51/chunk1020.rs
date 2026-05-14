//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1020/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1020<F: Float>(t11921: F, t247: F, t31920: F, t33754: F, t4930: F, t8513: F, t1035: F, t1042: F, t1043: F, t1089: F, t120256: F, t120387: F, t120419: F, t120443: F, t120447: F, t120558: F, t120584: F, t120654: F, t1669: F, t19639: F, t31928: F, t31966: F, t33756: F, t33770: F, t33796: F, t33797: F, t33803: F, t33832: F, t4866: F, t8517: F) -> (F, F) {
    let t126719 = t31920 * t247 * t11921 * t33754;
    let t126725 = t8513 * t4930;
    let t126741 = 0.3718732920905101082e-3 * t120256 * t1042 * t1669 * t19639 + 0.15058382271189582399e-2 * t31966 * t33770 + 0.30116764542379164799e-2 * t120419 * t33756 - 0.37645955677973955999e-3 * t126719 - 0.17135921299530705785e1 * t120584 * t33797 - 0.12395776403017003607e-3 * t120387 * t33832 + 0.57119737665102352616e0 * t126725 * t8517 - 0.8673628188205199462e0 * t31928 * t1035 * t4866 * t1089 - 0.3427184259906141157e1 * t120558 * t33796 * t1043 * t1089 - 0.3427184259906141157e1 * t120654 * t33803 * t1043 * t1089 + 0.66110807482757352571e-3 * t120443 + t120447;
    (t126725, t126741)
}

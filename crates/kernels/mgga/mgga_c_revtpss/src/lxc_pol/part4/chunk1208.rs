//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1208/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1208<F: Float>(t141: F, t16889: F, t5098: F, t698: F, t16725: F, t3417: F, t16729: F, t16720: F, t1145: F, t16738: F, t12254: F, t16715: F, t16708: F, t16710: F, t16712: F, t12296: F, t12297: F, t12299: F, t12301: F, t12303: F, t16706: F, t16717: F, t16722: F, t16727: F, t16731: F, t16735: F, t16740: F, t16744: F, t16748: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16890 = t141 * t16889;
    let t16892 = t698 * t5098;
    let t16893 = 0.21908444444444444444e0 * t16892;
    let t16894 = t3417 * t16725;
    let t16895 = t141 * t16894;
    let t16897 = t3417 * t16729;
    let t16898 = t141 * t16897;
    let t16900 = t3417 * t16720;
    let t16901 = t141 * t16900;
    let t16903 = t1145 * t16738;
    let t16904 = t141 * t16903;
    let t16907 = t12254 * t16715;
    let t16908 = t141 * t16907;
    let t16915 = 4.0 / 27.0 * t16708;
    let t16916 = 4.0 / 9.0 * t16710;
    let t16917 = 2.0 / 9.0 * t16712;
    let t16926 = -t12296 + 8.0 / 27.0 * t12297 + 2.0 / 27.0 * t12299 - 2.0 / 9.0 * t12301 - t12303 / 9.0 + 4.0 / 27.0 * t16706 + t16915 - t16916 - t16917 + 10.0 / 27.0 * t16717 - 4.0 / 3.0 * t16722 - 4.0 / 9.0 * t16727 - 2.0 / 9.0 * t16731 + 2.0 * t16735 + 4.0 / 3.0 * t16740 + 2.0 / 3.0 * t16744 + t16748 / 3.0;
    (t16890, t16892, t16893, t16895, t16898, t16901, t16904, t16908, t16926)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1327/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1327(t2186: f64, t2319: f64, t112: f64, t30217: f64, t1268: f64, t12725: f64, t12734: f64, t15857: f64, t19456: f64, t2180: f64, t2181: f64, t2183: f64, t2314: f64, t26114: f64, t26117: f64, t26179: f64, t29890: f64, t29935: f64, t29944: f64, t29947: f64, t29956: f64, t29963: f64, t30186: f64, t4028: f64, t5361: f64, t652: f64, t7458: f64, t7676: f64, t8124: f64, t8143: f64, t8144: f64, t8148: f64, t8150: f64, t8235: f64, t90375: f64, t90381: f64) -> (f64, f64, f64) {
    let t110671 = t2186 * t2319;
    let t110684 = t30217 * t112;
    let t110736 = 4.0_f64 * t26117 * t8150 + 2.0_f64 * t7676 * t29956 - 2.0_f64 * t652 * t15857 * t2180 + 2.0_f64 * t4028 * t29944 + 4.0_f64 * t2314 * t30186 - 2.0_f64 * t4028 * t29963 - 4.0_f64 * t26114 * t8144 - 4.0_f64 * t26179 * t8144 - 4.0_f64 * t7458 * t29890 - 4.0_f64 * t19456 * t8144 - 4.0_f64 * t12725 * t8124 - 2.0_f64 * t7458 * t29963 + 2.0_f64 * t90375 * t2183 + 4.0_f64 * t26117 * t8148 + 4.0_f64 * t4028 * t29947 - 2.0_f64 * t4028 * t29935 - 2.0_f64 * t90381 * t2181 + 2.0_f64 * t4028 * t29956 + 4.0_f64 * t1268 * t8143 * t5361 + 4.0_f64 * t12734 * t8235;
    (t110671, t110684, t110736)
}

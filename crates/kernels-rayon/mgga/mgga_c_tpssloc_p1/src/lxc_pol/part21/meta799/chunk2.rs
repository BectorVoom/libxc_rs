//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2781/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2781(t17013: f64, t9638: f64, t13258: f64, t16845: f64, t13261: f64, t4166: f64, t13151: f64, t13156: f64, t13164: f64, t13191: f64, t16723: f64, t16729: f64, t16737: f64, t16749: f64, t1891: f64, t228: f64, t2379: f64, t2667: f64, t2671: f64, t2675: f64, t4219: f64, t4225: f64, t4227: f64, t4230: f64, t5544: f64, t5601: f64, t5605: f64, t5608: f64, t58090: f64, t58139: f64, t68: f64, t822: f64, t824: f64, t825: f64) -> (f64, f64, f64, f64) {
    let t58890 = t9638 * t17013;
    let t58900 = t13258 * t16845;
    let t58904 = t4166 * t13261;
    let t58947 = 60.0_f64 * t1891 * t2379 * t4225 * t5544 + 240.0_f64 * t13156 * t13191 * t4225 - 24.0_f64 * t228 * t2671 * t58090 + 3.0_f64 * t228 * t58139 * t824 - 48.0_f64 * t4219 * t4227 * t68 + 120.0_f64 * t13151 * t16737 - 24.0_f64 * t13164 * t16729 + 6.0_f64 * t16723 * t825 + 6.0_f64 * t16749 * t822 - 12.0_f64 * t2667 * t5605 + 3.0_f64 * t2667 * t5608 + 3.0_f64 * t2675 * t5601 + 12.0_f64 * t4219 * t4230;
    (t58890, t58900, t58904, t58947)
}

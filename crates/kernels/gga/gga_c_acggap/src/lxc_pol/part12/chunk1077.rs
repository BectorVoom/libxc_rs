//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1077/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1077<F: Float>(t2131: F, t2147: F, t309: F, t9417: F, t463: F, t9431: F, t2132: F, t2138: F, t322: F, t9367: F, t8073: F, t8397: F, t2146: F, t33080: F, t33085: F, t33088: F, t33090: F, t33093: F, t33097: F, t33100: F, t33104: F, t33107: F, t33561: F, t7931: F, t8004: F, t8306: F) -> (F,) {
    let t38153 = 0.34694512752820797848e1 * t2131 * t2147 * t9417 * t309;
    let t38157 = 0.34694512752820797848e1 * t2131 * t2147 * t9431 * t463;
    let t38165 = 0.17347256376410398924e1 * t2138 * t2132 * t9367 * t322;
    let t38176 = 0.34694512752820797848e1 * t8397 * t8073;
    let t38178 = t38153 + t38157 - 0.8673628188205199462e0 * t7931 * t8306 * t33561 - 0.26341796731742046395e1 * t33080 + t38165 + 0.8673628188205199462e0 * t33085 - 0.8673628188205199462e0 * t33088 - 0.13170898365871023197e1 * t33090 - 0.13170898365871023197e1 * t33093 - 0.17347256376410398924e1 * t33097 - t33100 - 0.52041769129231196772e1 * t2146 * t8004 * t9417 * t463 + t38176 + 0.17347256376410398924e1 * t33104 - t33107;
    (t38178,)
}

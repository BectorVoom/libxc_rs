//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 513/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk513<F: Float>(t1034: F, t1044: F, t164: F, t167: F, t183: F, t2594: F, t2639: F, t2647: F, t2670: F, t2682: F, t2693: F, t588: F, t600: F, t621: F, t626: F, t1045: F, t1055: F, t184: F, t188: F, t2671: F, t2679: F, t622: F, t634: F) -> (F, F, F) {
    let t2702 = 0.13170898365871023197e1 * t2682 * t2594 - 0.65854491829355115987e0 * t588 * t621 * t1034 * t164 - 0.65854491829355115987e0 * t588 * t183 * t2639 * t164 - 0.65854491829355115987e0 * t2693 * t2647 - 0.65854491829355115987e0 * t588 * t1044 * t600 * t164 + 0.65854491829355115987e0 * t167 * t2670;
    let t2703 = t626 * t2702;
    let t2706 = 0.65854491829355115987e0 * t2671 * t188 - 0.65854491829355115987e0 * t1045 * t634 - 0.65854491829355115987e0 * t622 * t1055 + 0.13170898365871023197e1 * t184 * t2679 - 0.65854491829355115987e0 * t184 * t2703;
    (t2702, t2703, t2706)
}

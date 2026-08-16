//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 931/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk931(t183: f64, t5389: f64, t1717: f64, t621: f64, t588: f64, t1044: f64, t1719: f64, t1034: f64, t164: f64, t167: f64, t1721: f64, t1753: f64, t1783: f64, t2594: f64, t2639: f64, t2647: f64, t2670: f64, t2682: f64, t2693: f64, t600: f64, t6860: f64, t6865: f64, t6869: f64, t6881: f64, t6898: f64, t6903: f64, t6980: f64, t7084: f64, t7096: f64) -> (f64, f64, f64, f64) {
    let t7123 = t5389 * t183;
    let t7126 = t1717 * t621;
    let t7143 = t588 * t621;
    let t7156 = t1044 * t1719;
    let t7173 = -0.39512695097613069591e1_f64 * t7123 * t6898 + 0.26341796731742046394e1_f64 * t7126 * t2594 + 0.26341796731742046394e1_f64 * t2682 * t6865 + 0.39512695097613069591e1_f64 * t2682 * t6903 + 0.13170898365871023197e1_f64 * t2682 * t6869 - 0.65854491829355115987e0_f64 * t588 * t1783 * t1034 * t164 - 0.13170898365871023197e1_f64 * t588 * t621 * t2639 * t164 - 0.13170898365871023197e1_f64 * t7143 * t2647 - 0.65854491829355115987e0_f64 * t588 * t183 * t7084 * t164 - 0.13170898365871023197e1_f64 * t2693 * t6980 - 0.65854491829355115987e0_f64 * t2693 * t6860 - 0.65854491829355115987e0_f64 * t2693 * t6881 + 0.13170898365871023197e1_f64 * t1717 * t7156 * t1721 - 0.13170898365871023197e1_f64 * t588 * t2670 * t600 * t164 - 0.65854491829355115987e0_f64 * t588 * t1044 * t1753 * t164 - 0.65854491829355115987e0_f64 * t588 * t7156 * t164 + 0.65854491829355115987e0_f64 * t167 * t7096;
    (t7123, t7126, t7143, t7173)
}

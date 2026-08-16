//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 844/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk844(t633: f64, t9042: f64, t3410: f64, t621: f64, t1044: f64, t1717: f64, t588: f64, t1034: f64, t164: f64, t167: f64, t1721: f64, t183: f64, t2594: f64, t2639: f64, t2647: f64, t2670: f64, t2682: f64, t2693: f64, t3441: f64, t3460: f64, t600: f64, t7123: f64, t8888: f64, t8910: f64, t8920: f64, t8949: f64, t8954: f64, t8958: f64, t8967: f64, t8972: f64, t9019: f64) -> (f64, f64, f64, f64) {
    let t9043 = t9042 * t633;
    let t9048 = t621 * t3410;
    let t9056 = t1717 * t1044;
    let t9067 = t588 * t1044;
    let t9095 = -0.39512695097613069591e1_f64 * t7123 * t8954 + 0.13170898365871023197e1_f64 * t1717 * t9048 * t1721 + 0.26341796731742046394e1_f64 * t2682 * t8920 + 0.39512695097613069591e1_f64 * t2682 * t8958 + 0.26341796731742046394e1_f64 * t9056 * t2594 - 0.13170898365871023197e1_f64 * t588 * t2670 * t1034 * t164 - 0.13170898365871023197e1_f64 * t588 * t1044 * t2639 * t164 - 0.13170898365871023197e1_f64 * t9067 * t2647 + 0.13170898365871023197e1_f64 * t2682 * t8949 - 0.65854491829355115987e0_f64 * t588 * t621 * t3441 * t164 - 0.65854491829355115987e0_f64 * t588 * t183 * t8888 * t164 - 0.65854491829355115987e0_f64 * t2693 * t8910 - 0.65854491829355115987e0_f64 * t588 * t9048 * t164 - 0.13170898365871023197e1_f64 * t2693 * t8967 - 0.65854491829355115987e0_f64 * t2693 * t8972 - 0.65854491829355115987e0_f64 * t588 * t3460 * t600 * t164 + 0.65854491829355115987e0_f64 * t167 * t9019;
    (t9043, t9056, t9067, t9095)
}

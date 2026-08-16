//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1076/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1076(t1882: f64, t35653: f64, t35730: f64, t10002: f64, t110660: f64, t141713: f64, t141914: f64, t141942: f64, t141947: f64, t1424: f64, t149865: f64, t149997: f64, t151092: f64, t151407: f64, t1901: f64, t242: f64, t2469: f64, t2574: f64, t27836: f64, t27889: f64, t28140: f64, t28255: f64, t33771: f64, t35553: f64, t35599: f64, t35613: f64, t3821: f64, t3837: f64, t3842: f64, t446: f64, t51853: f64, t6074: f64, t6154: f64, t6187: f64, t6837: f64, t729: f64, t7553: f64, t7560: f64, t762: f64) -> f64 {
    let t151842 = t1882 * t35653;
    let t151855 = t1882 * t35730;
    let t151893 = 2.0_f64 / 3.0_f64 * t446 * t2574 * t7560 * t3837 + t446 * t729 * t762 * t7553 * t3821 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t242 * t149997 + t141914 / 9.0_f64 - 4.0_f64 / 9.0_f64 * t151842 - 2.0_f64 / 3.0_f64 * t446 * t729 * t10002 * t35599 + 4.0_f64 / 3.0_f64 * t446 * t242 * t151407 + t141942 + 2.0_f64 / 9.0_f64 * t141947 - t446 * t242 * t151092 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t151855 + 8.0_f64 * t1901 * t110660 * t33771 * t3837 + 2.0_f64 * t1901 * t28140 * t141713 * t3842 - 4.0_f64 * t1901 * t28140 * t6074 * t27836 + 2.0_f64 / 3.0_f64 * t446 * t729 * t762 * t6837 * t6187 - 4.0_f64 / 3.0_f64 * t1901 * t51853 * t35613 + 2.0_f64 / 3.0_f64 * t446 * t242 * t149865 + 2.0_f64 / 3.0_f64 * t446 * t729 * t6154 * t28255 + 2.0_f64 / 3.0_f64 * t446 * t729 * t762 * t1424 * t27889 + 2.0_f64 / 3.0_f64 * t446 * t729 * t2469 * t35553;
    t151893
}

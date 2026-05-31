//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1076/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1076<F: Float>(t1882: F, t35653: F, t35730: F, t10002: F, t110660: F, t141713: F, t141914: F, t141942: F, t141947: F, t1424: F, t149865: F, t149997: F, t151092: F, t151407: F, t1901: F, t242: F, t2469: F, t2574: F, t27836: F, t27889: F, t28140: F, t28255: F, t33771: F, t35553: F, t35599: F, t35613: F, t3821: F, t3837: F, t3842: F, t446: F, t51853: F, t6074: F, t6154: F, t6187: F, t6837: F, t729: F, t7553: F, t7560: F, t762: F) -> F {
    let t151842 = t1882 * t35653;
    let t151855 = t1882 * t35730;
    let t151893 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2574 * t7560 * t3837 + t446 * t729 * t762 * t7553 * t3821 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t242 * t149997 + t141914 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t151842 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t729 * t10002 * t35599 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t242 * t151407 + t141942 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t141947 - t446 * t242 * t151092 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t151855 + F::cast_from(8.0_f64) * t1901 * t110660 * t33771 * t3837 + F::cast_from(2.0_f64) * t1901 * t28140 * t141713 * t3842 - F::cast_from(4.0_f64) * t1901 * t28140 * t6074 * t27836 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t729 * t762 * t6837 * t6187 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t51853 * t35613 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t242 * t149865 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t729 * t6154 * t28255 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t729 * t762 * t1424 * t27889 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t729 * t2469 * t35553;
    t151893
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1250/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1250(t1985: f64, t26471: f64, t6889: f64, t6906: f64, t12020: f64, t120616: f64, t120621: f64, t120628: f64, t120629: f64, t120633: f64, t120641: f64, t1323: f64, t2016: f64, t22670: f64, t26224: f64, t26225: f64, t26481: f64, t31117: f64, t31189: f64, t32686: f64, t32690: f64, t32726: f64, t3758: f64, t3882: f64, t5215: f64, t5325: f64, t5326: f64, t568: f64, t7729: f64, t8485: f64, t91441: f64) -> f64 {
    let t120649 = 0.16449340668482264365e-1_f64 * t1985 * t6889 * t6906 * t26471;
    let t120652 = -6.0_f64 * t12020 * t26224 * t5325 * t8485 + t1323 * t32726 * t568 - 12.0_f64 * t26224 * t26225 * t26481 - 2.0_f64 * t2016 * t91441 + 4.0_f64 * t22670 * t7729 - 6.0_f64 * t31117 * t5215 + 2.0_f64 * t31189 * t5326 + 2.0_f64 * t32686 * t3882 - 6.0_f64 * t32690 * t3758 - t120616 - t120621 + t120628 + t120629 + t120633 - t120641 - t120649;
    t120652
}

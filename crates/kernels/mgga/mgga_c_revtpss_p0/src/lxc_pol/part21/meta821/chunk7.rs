//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3045/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3045<F: Float>(t30: F, t265: F, t393: F, t51814: F, t52167: F, t52197: F, t52227: F, t52870: F, t52883: F, t52906: F, t52924: F, t56115: F, t10326: F, t1106: F, t11095: F, t12201: F, t13312: F, t1468: F, t1469: F, t15083: F, t1587: F, t16618: F, t1704: F, t2257: F, t2258: F, t3340: F, t395: F, t4186: F, t45: F, t4560: F, t49889: F, t5028: F, t51827: F, t51829: F, t51831: F, t51833: F, t51835: F, t605: F, t606: F, t9344: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t56119 = piecewise3::<F>(t394, t52167 + t52197 + t52227 + t52870 + t52883 + t52906 + t52924 + t56115, t51814);
    let t56137 = piecewise3::<F>(t120, t51814 * t30 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t15083 * t605 + F::new(3.0) / F::new(2.0) * t4560 * t2257 + t1587 * t9344 / F::new(2.0) + t11095 * t1468 / F::new(2.0) + t51827 + t51829 - t51831 - t51833 + t51835, t56119 * t45 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t16618 * t606 + F::new(3.0) / F::new(2.0) * t5028 * t2258 + t1704 * t10326 / F::new(2.0) + t12201 * t1469 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t3340 * t4186 + F::new(3.0) / F::new(2.0) * t1106 * t13312 + t395 * t49889 / F::new(2.0));
    t56137
}

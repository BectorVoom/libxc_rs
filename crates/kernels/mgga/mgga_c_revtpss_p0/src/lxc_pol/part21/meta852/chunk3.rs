//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3205/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3205<F: Float>(t1248: F, t12776: F, t12835: F, t12866: F, t13053: F, t13063: F, t17353: F, t17513: F, t17654: F, t17661: F, t17662: F, t2251: F, t3367: F, t3604: F, t3630: F, t3720: F, t44510: F, t44561: F, t44578: F, t44769: F, t44886: F, t44888: F, t44892: F, t44898: F, t44902: F, t44906: F, t44912: F, t45371: F, t5341: F, t5354: F, t56999: F, t58909: F, t59062: F, t59066: F, t59078: F, t59094: F, t59096: F) -> F {
    let t59108 = -F::cast_from(0.42874018118069736972e-3_f64) * t44886 - F::cast_from(0.42874018118069736972e-3_f64) * t44888 - F::cast_from(0.14291339372689912324e-3_f64) * t44892 + F::cast_from(0.85748036236139473944e-3_f64) * t44561 * t17662 + F::cast_from(0.85748036236139473944e-3_f64) * t12866 * t59062 * t3630 + F::cast_from(0.25724410870841842184e-2_f64) * t59066 * t17353 * t13053 * t56999 - F::cast_from(0.17149607247227894789e-2_f64) * t17654 * t17353 * t3604 * t1248 * t3367 * t2251 + F::cast_from(0.57165357490759649295e-3_f64) * t59078 + F::cast_from(0.17149607247227894789e-2_f64) * t44510 * t58909 * t5341 * t17513 - F::cast_from(0.42344709252414555035e-3_f64) * t44898 + F::cast_from(0.42874018118069736972e-3_f64) * t12866 * t17661 * t12835 + F::cast_from(0.42874018118069736972e-3_f64) * t12866 * t17661 * t12776 + F::cast_from(0.95275595817932748827e-4_f64) * t44902 + F::cast_from(0.19055119163586549765e-3_f64) * t44906 - F::cast_from(0.85748036236139473944e-3_f64) * t59094 + F::cast_from(0.12862205435420921092e-2_f64) * t44578 * t3720 * t59096 * t13053 - F::cast_from(0.21437009059034868486e-3_f64) * t45371 * t3720 * t59096 * t13063 - F::cast_from(0.64311027177104605458e-3_f64) * t44769 * t5354 - t44912 / F::cast_from(144.0_f64);
    t59108
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3774/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3774<F: Float>(t5245: F, t5457: F, t3601: F, t6695: F, t1234: F, t12717: F, t1280: F, t1287: F, t1291: F, t13133: F, t16775: F, t17289: F, t17454: F, t17846: F, t21333: F, t21484: F, t21513: F, t21610: F, t3666: F, t3670: F, t3755: F, t3782: F, t3783: F, t45654: F, t45659: F, t45683: F, t45715: F, t490: F, t5346: F, t5452: F, t5486: F, t59650: F, t6573: F, t6587: F, t69655: F, t70422: F, t71179: F, t71724: F, t72044: F, t72087: F, t73: F) -> (F, F) {
    let t72143 = t5457 * t5245;
    let t72165 = t6695 * t3601;
    let t72187 = F::cast_from(0.52683593463484092788e1_f64) * t12717 * t5346 * t73 * t72143 - F::cast_from(0.52683593463484092788e1_f64) * t45715 * t21513 - F::cast_from(0.13170898365871023197e1_f64) * t3755 * t71724 * t1287 - F::cast_from(0.26341796731742046394e1_f64) * t45683 * t21484 + F::cast_from(0.65854491829355115987e0_f64) * t71179 * t490 - F::cast_from(0.26341796731742046394e1_f64) * t17289 * t5452 - F::cast_from(0.26341796731742046394e1_f64) * t3666 * t21610 + F::cast_from(0.13170898365871023197e1_f64) * t21333 * t1291 + F::cast_from(0.13170898365871023197e1_f64) * t3670 * t1280 * t70422 - F::cast_from(0.65854491829355115987e0_f64) * t3782 * t72165 * t3783 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t13133 * t6587 + F::cast_from(0.26341796731742046394e1_f64) * t3670 * t5486 * t16775 + F::cast_from(0.13170898365871023197e1_f64) * t3670 * t13133 * t6573 - F::cast_from(0.79025390195226139182e1_f64) * t45654 * t69655 * t72044 + F::cast_from(0.15805078039045227836e2_f64) * t17846 * t59650 * t72087 + F::cast_from(0.79025390195226139182e1_f64) * t45659 * t69655 * t17454;
    (t72165, t72187)
}

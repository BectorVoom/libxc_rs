//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1104/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1104(t12297: f64, t12298: f64, t12301: f64, t151: f64, t18914: f64, t18916: f64, t18918: f64, t18920: f64, t18925: f64, t19775: f64, t19780: f64, t19783: f64, t19785: f64, t19792: f64, t19795: f64, t945: f64) -> f64 {
    let t19797 = 0.13170898365871023197e1_f64 * t151 * t19775 * t945 + 0.26341796731742046394e1_f64 * t19780 + 0.13170898365871023197e1_f64 * t19783 - 0.26341796731742046394e1_f64 * t19785 + 0.13170898365871023197e1_f64 * t18914 + t12297 - 0.26341796731742046394e1_f64 * t18916 - 0.26341796731742046394e1_f64 * t18918 + 0.13170898365871023197e1_f64 * t18920 - 0.39512695097613069592e1_f64 * t12298 - t12301 - 0.13170898365871023197e1_f64 * t19792 + 0.52683593463484092788e1_f64 * t18925 + 0.79025390195226139182e1_f64 * t19795;
    t19797
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1104/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1104<F: Float>(t12297: F, t12298: F, t12301: F, t151: F, t18914: F, t18916: F, t18918: F, t18920: F, t18925: F, t19775: F, t19780: F, t19783: F, t19785: F, t19792: F, t19795: F, t945: F) -> F {
    let t19797 = F::cast_from(0.13170898365871023197e1_f64) * t151 * t19775 * t945 + F::cast_from(0.26341796731742046394e1_f64) * t19780 + F::cast_from(0.13170898365871023197e1_f64) * t19783 - F::cast_from(0.26341796731742046394e1_f64) * t19785 + F::cast_from(0.13170898365871023197e1_f64) * t18914 + t12297 - F::cast_from(0.26341796731742046394e1_f64) * t18916 - F::cast_from(0.26341796731742046394e1_f64) * t18918 + F::cast_from(0.13170898365871023197e1_f64) * t18920 - F::cast_from(0.39512695097613069592e1_f64) * t12298 - t12301 - F::cast_from(0.13170898365871023197e1_f64) * t19792 + F::cast_from(0.52683593463484092788e1_f64) * t18925 + F::cast_from(0.79025390195226139182e1_f64) * t19795;
    t19797
}

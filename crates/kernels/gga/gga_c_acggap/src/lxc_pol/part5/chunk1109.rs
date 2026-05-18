//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1109/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1109<F: Float>(t1922: F, t980: F, t381: F, t6454: F, t879: F, t1170: F, t12326: F, t12328: F, t12344: F, t18989: F, t19000: F, t19757: F, t19862: F, t19864: F, t19870: F, t19874: F, t19880: F, t407: F, t6465: F, t930: F) -> F {
    let t19882 = t980 * t1922;
    let t19885 = t381 * t6454 * t879;
    let t19887 = -F::new(0.65854491829355115987e0) * t1170 * t6465 * t930 + F::new(0.65854491829355115987e0) * t19862 + F::new(0.13170898365871023197e1) * t19864 + F::new(0.13170898365871023197e1) * t12326 - F::new(0.39512695097613069592e1) * t12328 + F::new(0.13170898365871023197e1) * t18989 + F::new(0.26341796731742046394e1) * t19870 + F::new(0.13170898365871023197e1) * t19874 - F::new(0.26341796731742046394e1) * t1170 * t19757 * t407 + F::new(0.52683593463484092788e1) * t19000 + F::new(0.26341796731742046394e1) * t19880 + F::new(0.13170898365871023197e1) * t19882 + t12344 - F::new(0.65854491829355115987e0) * t19885;
    t19887
}

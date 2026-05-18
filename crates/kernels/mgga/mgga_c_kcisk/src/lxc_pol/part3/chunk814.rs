//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 814/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk814<F: Float>(t12535: F, t2918: F, t848: F, t2900: F, t846: F, t3346: F, t12489: F, t12491: F, t12493: F, t12524: F, t12526: F, t12528: F, t5680: F, t5744: F) -> (F, F, F) {
    let t12537 = t2918 * t12535 * t848;
    let t12540 = t2900 * t846;
    let t12541 = t12540 * t3346;
    let t12552 = -F::new(0.34523333333333333333e1) * t12489 + F::new(0.23015555555555555556e1) * t12491 - F::new(0.26851481481481481482e1) * t12493 - F::new(0.93932222222222222223e0) * t5680 + F::new(0.73355e-1) * t12524 - F::new(0.14671e0) * t12526 - F::new(0.17116166666666666667e0) * t12528 - F::new(0.36793333333333333333e0) * t5744;
    (t12537, t12541, t12552)
}

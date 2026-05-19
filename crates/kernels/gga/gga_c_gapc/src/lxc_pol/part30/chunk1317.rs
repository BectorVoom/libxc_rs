//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1317/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1317<F: Float>(t34638: F, t34641: F, t34644: F, t34647: F, t34651: F, t34654: F, t34658: F, t34661: F, t34663: F, t34666: F, t34669: F, t34673: F, t34676: F, t34679: F, t34682: F, t34686: F, t34689: F, t34692: F, t34695: F, t34698: F, t34701: F, t34704: F) -> (F, F) {
    let t38232 = -F::cast_from(0.86880925264517213544e-4_f64) * t34638 - F::cast_from(0.14391604187239777452e-6_f64) * t34641 + F::cast_from(0.21587406280859666178e-5_f64) * t34644 + F::cast_from(0.19191204183684243232e-6_f64) * t34647 + F::cast_from(0.19666550313313802087e-7_f64) * t34651 + F::cast_from(0.98332751566569010433e-8_f64) * t34654 - F::cast_from(0.13111033542209201391e-7_f64) * t34658 - F::cast_from(0.43440462632258606772e-4_f64) * t34661 - F::cast_from(0.22745373045674261828e-5_f64) * t34663 + F::cast_from(0.10984838052999936404e-3_f64) * t34666 + F::cast_from(0.9275345110817126956e-4_f64) * t34669;
    let t38244 = F::cast_from(0.13900948042322754167e-2_f64) * t34673 - F::cast_from(0.18550690221634253912e-3_f64) * t34676 + F::cast_from(0.13900948042322754167e-2_f64) * t34679 - F::cast_from(0.70531721015421066817e-5_f64) * t34682 - F::cast_from(0.9275345110817126956e-4_f64) * t34686 - F::cast_from(0.43284943850479925794e-3_f64) * t34689 + F::cast_from(0.14659167650695868203e-4_f64) * t34692 + F::cast_from(0.17679409834076461864e-6_f64) * t34695 - F::cast_from(0.94854674673349911132e-9_f64) * t34698 + F::cast_from(0.13259557375557346398e-6_f64) * t34701 - F::cast_from(0.58999650939941406261e-7_f64) * t34704;
    (t38232, t38244)
}

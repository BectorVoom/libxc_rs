//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1098/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1098<F: Float>(t1220: F, t1914: F, t316: F, t879: F, t1937: F, t449: F, t863: F, t864: F, t14620: F, t14621: F, t14626: F, t14640: F, t14642: F, t14648: F, t15259: F, t15262: F, t15265: F, t15276: F, t15278: F, t1608: F, t19664: F, t19668: F, t19672: F, t19676: F, t19678: F, t4109: F, t5332: F) -> F {
    let t19688 = t316 * t1220 * t1914 * t879;
    let t19692 = t863 * t449 * t1937 * t864;
    let t19696 = t14620 + F::new(0.52683593463484092788e1) * t15259 - F::new(0.65854491829355115987e0) * t19664 + F::new(0.13170898365871023197e1) * t14621 + t14626 - F::new(0.13170898365871023197e1) * t19668 - F::new(0.26341796731742046394e1) * t19672 - F::new(0.26341796731742046394e1) * t15262 - F::new(0.79025390195226139182e1) * t15265 - F::new(0.79025390195226139182e1) * t19676 - t14640 - F::new(0.13170898365871023197e1) * t19678 - F::new(0.13170898365871023197e1) * t14642 - F::new(0.79025390195226139182e1) * t1608 * t4109 + F::new(0.26341796731742046394e1) * t15276 + F::new(0.39512695097613069591e1) * t15278 + F::new(0.13170898365871023197e1) * t14648 - F::new(0.13170898365871023197e1) * t19688 - F::new(0.13170898365871023197e1) * t19692 - F::new(0.13170898365871023197e1) * t1608 * t5332;
    t19696
}

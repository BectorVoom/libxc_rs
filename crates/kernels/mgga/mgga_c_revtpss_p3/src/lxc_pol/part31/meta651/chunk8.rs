//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2160/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2160<F: Float>(t107201: F, t1096: F, t1695: F, t1983: F, t1984: F, t20152: F, t20188: F, t25591: F, t25692: F, t27419: F, t27550: F, t27576: F, t27616: F, t27679: F, t27699: F, t27703: F, t29747: F, t29875: F, t29887: F, t359: F, t4773: F, t4947: F, t5015: F, t5016: F, t6259: F, t7140: F, t7144: F, t7145: F, t7151: F, t7160: F, t7821: F, t94026: F, t988: F, t999: F, t99934: F) -> F {
    let t107206 = F::cast_from(0.34694512752820797848e1_f64) * t7144 * t7160 * t29887 * t988 - F::cast_from(0.34694512752820797848e1_f64) * t7151 * t7160 * t7821 * t5015 - F::cast_from(0.65854491829355115987e0_f64) * t7140 * t20152 + F::cast_from(0.34694512752820797848e1_f64) * t99934 * t27703 + F::cast_from(0.34694512752820797848e1_f64) * t7144 * t7160 * t29747 * t1096 - F::cast_from(0.13170898365871023197e1_f64) * t27550 * t4773 + F::cast_from(0.17347256376410398924e1_f64) * t25591 * t7145 * t29875 * t999 - F::cast_from(0.13170898365871023197e1_f64) * t27616 * t5016 + F::cast_from(0.26341796731742046394e1_f64) * t27616 * t4947 - F::cast_from(0.39512695097613069591e1_f64) * t94026 * t20188 + F::cast_from(0.34694512752820797848e1_f64) * t7144 * t7160 * t27679 * t1695 - F::cast_from(0.65854491829355115987e0_f64) * t25692 * t6259 + F::cast_from(0.26341796731742046394e1_f64) * t27699 * t4947 - F::cast_from(0.34694512752820797848e1_f64) * t27419 * t27576 - F::cast_from(0.4336814094102599731e0_f64) * t1983 * t1984 * t359 * t107201;
    t107206
}

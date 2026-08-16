//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1590/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1590<F: Float>(t1089: F, t19829: F, t19836: F, t1024: F, t1087: F, t12146: F, t12149: F, t12154: F, t15670: F, t19608: F, t19612: F, t19617: F, t19856: F, t20113: F, t20119: F, t20123: F, t20128: F, t20133: F, t20136: F, t3204: F, t3278: F, t3287: F, t342: F, t381: F, t4961: F, t4999: F, t6365: F, t6379: F, t6389: F, t989: F) -> F {
    let t20139 = t19829 * t1089;
    let t20146 = t19836 * t1089;
    let t20149 = -F::cast_from(0.13170898365871023197e1_f64) * t19608 * t4999 - F::cast_from(0.65854491829355115987e0_f64) * t3287 * t19612 + F::cast_from(0.65854491829355115987e0_f64) * t989 * t6389 + F::cast_from(0.13170898365871023197e1_f64) * t3204 * t19617 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t20113 + F::cast_from(0.65854491829355115987e0_f64) * t19856 * t381 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t20119 + F::cast_from(0.13170898365871023197e1_f64) * t1087 * t20123 + F::cast_from(0.13170898365871023197e1_f64) * t3278 * t6379 - F::cast_from(0.13170898365871023197e1_f64) * t1024 * t20128 + F::cast_from(0.26341796731742046394e1_f64) * t15670 * t4961 - F::cast_from(0.13170898365871023197e1_f64) * t1024 * t20133 - F::cast_from(0.13170898365871023197e1_f64) * t3287 * t20136 + F::cast_from(0.13170898365871023197e1_f64) * t12149 * t20139 - F::cast_from(0.13170898365871023197e1_f64) * t12146 * t6365 - F::cast_from(0.13170898365871023197e1_f64) * t12154 * t6365 - F::cast_from(0.13170898365871023197e1_f64) * t3287 * t20146;
    t20149
}

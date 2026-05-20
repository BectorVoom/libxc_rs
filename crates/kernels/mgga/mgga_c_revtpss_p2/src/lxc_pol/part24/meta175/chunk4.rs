//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 868/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk868<F: Float>(t1437: F, t1883: F, t213: F, t4082: F, t4085: F, t4099: F, t4113: F, t4114: F, t546: F, t5738: F, t5742: F, t5761: F, t5765: F, t5767: F, t6844: F, t6862: F, t6874: F, t6888: F, t820: F) -> F {
    let t6918 = t4082 - t4085 + F::cast_from(0.10975748638225852664e-1_f64) * t5738 - F::cast_from(0.10975748638225852664e-1_f64) * t5761 + t4099 - F::cast_from(0.19514881078765566038e-1_f64) * t5742 + F::cast_from(0.19514881078765566038e-1_f64) * t5765 - t4113 + F::cast_from(0.13170898365871023197e1_f64) * t820 * t4114 * t6862 - F::cast_from(0.13170898365871023197e1_f64) * t820 * t5767 * t1883 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t1437 * t6844 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t1437 * t6874 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t546 * t6888;
    t6918
}

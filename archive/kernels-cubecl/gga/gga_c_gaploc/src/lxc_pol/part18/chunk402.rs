//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 402/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk402<F: Float>(t656: F, t90: F, t256: F, t64: F, t1194: F, t1199: F, t1201: F, t1206: F, t408: F, t1741: F, t1762: F, t1832: F, t257: F, t260: F, t266: F, t657: F, t667: F, t670: F, t677: F) -> (F, F) {
    let t1913 = t90 * t656;
    let t1916 = t256 * t256;
    let t1917 = F::cast_from(1.0_f64) / t1916;
    let t1918 = t64 * t1917;
    let t1931 = -F::cast_from(0.15474205398478635379e-1_f64) * t408 + F::cast_from(0.5833205e-2_f64) * t1194 - F::cast_from(0.16123583333333333333e-2_f64) * t1199 + F::cast_from(0.61251011229312867192e-4_f64) * t1201 - F::cast_from(0.6735290625e-5_f64) * t1206;
    let t1933 = F::cast_from(0.21272952746160294864e-2_f64) * t408 * t257 + F::cast_from(0.42545905492320589728e-2_f64) * t1913 * t667 + F::cast_from(0.63818858238480884592e-2_f64) * t1918 * t1741 - F::cast_from(0.21272952746160294864e-2_f64) * t657 * t1762 - t1832 * t266 - F::cast_from(2.0_f64) * t670 * t677 - t260 * t1931;
    (t1931, t1933)
}

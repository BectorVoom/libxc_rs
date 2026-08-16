//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1079/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1079(t2268: f64, t4500: f64, t2264: f64, t4479: f64, t4481: f64, t1628: f64, t7996: f64, t12930: f64, t12933: f64, t12940: f64, t1629: f64, t1636: f64, t27498: f64, t27502: f64, t27693: f64, t4475: f64, t4480: f64, t7998: f64, t8001: f64, t8010: f64) -> (f64, f64, f64, f64, f64) {
    let t27697 = t2268 * t4500;
    let t27702 = t2264 * t4479;
    let t27705 = t2268 * t4481;
    let t27710 = t7996 * t1628;
    let t27713 = -t12930 * t2268 + 4.0_f64 * t12933 * t8001 - 6.0_f64 * t12940 * t27705 - t1629 * t27693 - 2.0_f64 * t1636 * t27710 + 2.0_f64 * t27697 * t4480 + 2.0_f64 * t27702 * t4481 - 2.0_f64 * t4475 * t8010 - t4500 * t7998 + t27498 + t27502;
    (t27697, t27702, t27705, t27710, t27713)
}

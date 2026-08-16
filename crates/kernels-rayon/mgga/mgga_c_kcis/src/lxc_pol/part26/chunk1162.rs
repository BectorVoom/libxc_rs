//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1162/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1162(t29458: f64, t29486: f64, t1506: f64, t2268: f64, t7566: f64, t2128: f64, t8251: f64, t7537: f64, t12940: f64, t18268: f64, t23255: f64, t27702: f64, t28649: f64, t29413: f64, t29421: f64, t4480: f64, t6222: f64, t7998: f64, t8240: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29487 = t29458 + t29486;
    let t29488 = t1506 * t29487;
    let t29489 = t2268 * t7566;
    let t29499 = t8251 * t2128;
    let t29502 = t2268 * t7537;
    let t29508 = -6.0_f64 * t12940 * t29502 + 4.0_f64 * t18268 * t8240 - 2.0_f64 * t2128 * t28649 - t2268 * t23255 + 2.0_f64 * t27702 * t7537 + 2.0_f64 * t29489 * t4480 + 4.0_f64 * t29499 * t4480 - 2.0_f64 * t6222 * t8251 - t7566 * t7998 - t29413 - t29421;
    (t29487, t29488, t29489, t29499, t29502, t29508)
}

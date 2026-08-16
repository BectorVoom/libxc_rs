//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1283/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1283(t12930: f64, t12933: f64, t12940: f64, t1636: f64, t18352: f64, t18355: f64, t2268: f64, t27702: f64, t28658: f64, t28666: f64, t4480: f64, t4500: f64, t8240: f64, t8251: f64, t97961: f64, t97976: f64, t97977: f64, t97979: f64, t97984: f64, t97989: f64, t97990: f64, t97993: f64, t97996: f64, t98956: f64) -> f64 {
    let t98957 = -12.0_f64 * t12940 * t1636 * t28658 - 6.0_f64 * t12940 * t4500 * t8240 + 2.0_f64 * t18352 * t2268 * t4480 + 2.0_f64 * t4480 * t4500 * t8251 - t12930 * t8251 + 4.0_f64 * t12933 * t28658 + 4.0_f64 * t12933 * t28666 + 2.0_f64 * t18355 * t27702 + t97961 + t97976 + t97977 + t97979 + t97984 - t97989 + t97990 - t97993 - t97996 - t98956;
    t98957
}

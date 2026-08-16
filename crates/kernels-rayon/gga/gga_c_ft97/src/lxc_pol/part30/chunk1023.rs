//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1023/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1023(t150522: f64, t17836: f64, t226: f64, t7447: f64, t173: f64, t35414: f64, t27521: f64, t7470: f64, t1095: f64, t123607: f64, t13520: f64, t140937: f64, t1418: f64, t150496: f64, t150500: f64, t150512: f64, t150517: f64, t17806: f64, t27566: f64, t27730: f64, t33356: f64, t33357: f64, t33368: f64, t33372: f64, t33434: f64, t33436: f64, t3751: f64, t3791: f64, t3817: f64, t683: f64, t7590: f64, t79529: f64) -> (f64, f64) {
    let t150523 = t17836 * t150522;
    let t150526 = t7447 * t226;
    let t150533 = t173 * t35414;
    let t150535 = t27521 * t7470 * t150533;
    let t150537 = 0.15322466011111111111e0_f64 * t33372 * t1418 * t150496 - 4.0_f64 * t150500 * t27730 + 0.13359406463155864749e-8_f64 * t79529 * t17806 * t7590 * t1095 - 0.17608347349624143343e-1_f64 * t33434 * t33436 * t683 * t3817 + 0.25845121844514357744e-4_f64 * t140937 * t150512 - 0.61277550024922479209e-6_f64 * t123607 * t150517 - 0.25845121844514357744e-4_f64 * t13520 * t150512 + 0.89080607335887169333e-3_f64 * t150523 * t33368 + 4.0_f64 * t150526 * t3791 + 0.10338048737805743097e-3_f64 * t27566 * t33356 * t33357 * t3751 - 0.22705522127871165896e-3_f64 * t150535;
    (t150533, t150537)
}

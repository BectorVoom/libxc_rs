//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1023/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1023<F: Float>(t150522: F, t17836: F, t226: F, t7447: F, t173: F, t35414: F, t27521: F, t7470: F, t1095: F, t123607: F, t13520: F, t140937: F, t1418: F, t150496: F, t150500: F, t150512: F, t150517: F, t17806: F, t27566: F, t27730: F, t33356: F, t33357: F, t33368: F, t33372: F, t33434: F, t33436: F, t3751: F, t3791: F, t3817: F, t683: F, t7590: F, t79529: F) -> (F, F) {
    let t150523 = t17836 * t150522;
    let t150526 = t7447 * t226;
    let t150533 = t173 * t35414;
    let t150535 = t27521 * t7470 * t150533;
    let t150537 = F::cast_from(0.15322466011111111111e0_f64) * t33372 * t1418 * t150496 - F::cast_from(4.0_f64) * t150500 * t27730 + F::cast_from(0.13359406463155864749e-8_f64) * t79529 * t17806 * t7590 * t1095 - F::cast_from(0.17608347349624143343e-1_f64) * t33434 * t33436 * t683 * t3817 + F::cast_from(0.25845121844514357744e-4_f64) * t140937 * t150512 - F::cast_from(0.61277550024922479209e-6_f64) * t123607 * t150517 - F::cast_from(0.25845121844514357744e-4_f64) * t13520 * t150512 + F::cast_from(0.89080607335887169333e-3_f64) * t150523 * t33368 + F::cast_from(4.0_f64) * t150526 * t3791 + F::cast_from(0.10338048737805743097e-3_f64) * t27566 * t33356 * t33357 * t3751 - F::cast_from(0.22705522127871165896e-3_f64) * t150535;
    (t150533, t150537)
}

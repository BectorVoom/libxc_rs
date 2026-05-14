//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1171/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1171<F: Float>(t33941: F, t33943: F, t33946: F, t33949: F, t33952: F, t33956: F, t33962: F, t33967: F, t33972: F, t33975: F, t33978: F, t33980: F, t33983: F, t33988: F, t33991: F, t33998: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36767 = 0.33147827249531850013e-7 * t33941;
    let t36768 = 0.66295654499063700026e-7 * t33943;
    let t36769 = 0.33147827249531850013e-7 * t33946;
    let t36770 = 0.13913017666225690434e-3 * t33949;
    let t36771 = 0.69504740211613770836e-3 * t33952;
    let t36773 = 0.67402122125348062552e-7 * t33956;
    let t36774 = 0.20041830772435757309e-6 * t33962;
    let t36775 = 0.83645744500336823644e-8 * t33967;
    let t36777 = 0.2318836277704281739e-4 * t33972;
    let t36778 = 0.71696352428860134552e-9 * t33975;
    let t36779 = 0.47797568285906756368e-9 * t33978;
    let t36780 = 0.11594181388521408695e-4 * t33980;
    let t36781 = 0.27312896163375289353e-9 * t33983;
    let t36782 = 0.49755503537412447748e-6 * t33988;
    let t36783 = 0.18310351929594268994e-5 * t33991;
    let t36788 = 0.71158605186385727883e-8 * t33998;
    (t36767, t36768, t36769, t36770, t36771, t36773, t36774, t36775, t36777, t36778, t36779, t36780, t36781, t36782, t36783, t36788)
}

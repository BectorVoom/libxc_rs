//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 832/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk832<F: Float>(t9048: F, t9051: F, t9054: F, t9057: F, t9062: F, t9064: F, t9069: F, t9073: F, t9076: F, t9081: F, t9085: F, t9088: F, t9093: F, t9097: F, t9100: F, t9104: F, t9106: F, t9108: F, t9111: F, t9115: F, t9118: F, t9121: F, t9124: F, t9126: F, t9130: F, t9132: F) -> (F, F) {
    let t10693 = 0.11255061864162936194e-7 * t9048 + 0.11255061864162936194e-6 * t9051 + 0.66704999981605668513e-8 * t9054 - 0.34752370105806885418e-3 * t9057 + 0.51564945349389680439e-8 * t9062 - 0.9275345110817126956e-4 * t9064 - 0.84540905957968605064e-6 * t9069 + 0.33765185592488808582e-6 * t9073 + 0.67530371184977617164e-6 * t9076 + 0.33765185592488808582e-6 * t9081 - 0.34752370105806885418e-3 * t9085 + 0.51491428373437201896e-5 * t9088 - 0.35580446990188463585e-8 * t9093;
    let t10708 = -0.33816362383187442026e-4 * t9097 + 0.28985453471303521736e-5 * t9100 - 0.91551759647971344971e-6 * t9104 + 0.33816362383187442026e-4 * t9106 - 0.10136107947527008247e-3 * t9108 - 0.10136107947527008247e-3 * t9111 - 0.37516872880543120646e-8 * t9115 + 0.25294579912893309636e-8 * t9118 + 0.12974218172834570556e-1 * t9121 - 0.27801896084645508334e-2 * t9124 + 0.132681342766433194e-5 * t9126 + 0.20241536458333333336e-3 * t9130 + 0.55603792169291016668e-2 * t9132;
    (t10693, t10708)
}

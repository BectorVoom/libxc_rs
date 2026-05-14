//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 832/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk832<F: Float>(t8972: F, t8974: F, t8976: F, t8978: F, t8980: F, t8982: F, t8984: F, t8988: F, t8990: F, t8994: F, t8996: F, t9000: F, t9003: F, t9009: F, t9011: F, t9014: F, t9017: F, t9021: F, t9024: F, t9027: F, t9032: F, t9034: F, t9036: F, t9038: F, t9042: F, t9044: F) -> (F, F) {
    let t10662 = 0.30353495895471971564e-6 * t8972 - 0.53968515702149165441e-6 * t8974 + 0.43284943850479925795e-3 * t8976 - 0.43284943850479925795e-3 * t8978 - 0.41223756048076119805e-5 * t8980 + 0.73295838253479341016e-5 * t8982 + 0.24761136101158459626e-5 * t8984 + 0.16009199995585360443e-6 * t8988 + 0.55603792169291016668e-2 * t8990 + 0.18550690221634253912e-3 * t8994 + 0.15458908518028544927e-5 * t8996 - 0.2748593934505475288e-5 * t9000 - 0.2471588561924985691e-3 * t9003;
    let t10679 = -0.2471588561924985691e-3 * t9009 - 0.36652500116630512966e-6 * t9011 - 0.55603792169291016668e-2 * t9014 + 0.15176747947735985782e-5 * t9017 - 0.2698425785107458272e-5 * t9021 - 0.15176747947735985782e-6 * t9024 + 0.2698425785107458272e-6 * t9027 + 0.14648281543675415196e-4 * t9032 - 0.4637672555408563478e-4 * t9034 + 0.11272120794395814009e-6 * t9036 - 0.20041830772435757309e-6 * t9038 + 0.11255061864162936194e-7 * t9042 + 0.11255061864162936194e-6 * t9044;
    (t10662, t10679)
}

//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1292/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1292<F: Float>(t35966: F, t35970: F, t35973: F, t35976: F, t35979: F, t35983: F, t35986: F, t35989: F, t35992: F, t35996: F, t35999: F, t36003: F, t36006: F) -> F {
    let t37599 = F::cast_from(0.58714905980103539484e-5_f64) * t35966 - F::cast_from(0.93943849568165663176e-4_f64) * t35970 + F::cast_from(0.29357452990051769742e-5_f64) * t35973 - F::cast_from(0.3757753982726626527e-3_f64) * t35976 + F::cast_from(0.19571635326701179828e-6_f64) * t35979 + F::cast_from(0.13298999885998441252e-6_f64) * t35983 - F::cast_from(0.93943849568165663176e-5_f64) * t35986 - F::cast_from(0.93943849568165663176e-5_f64) * t35989 + F::cast_from(0.55677388177399516375e-6_f64) * t35992 + F::cast_from(0.61871497612135212571e-7_f64) * t35996 + F::cast_from(0.65659062294300875668e-4_f64) * t35999 - F::cast_from(0.34798367610874697734e-7_f64) * t36003 - F::cast_from(0.65659062294300875668e-4_f64) * t36006;
    t37599
}

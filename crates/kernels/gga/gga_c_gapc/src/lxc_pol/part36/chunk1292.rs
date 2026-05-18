//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1292/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1292<F: Float>(t35966: F, t35970: F, t35973: F, t35976: F, t35979: F, t35983: F, t35986: F, t35989: F, t35992: F, t35996: F, t35999: F, t36003: F, t36006: F) -> F {
    let t37599 = F::new(0.58714905980103539484e-5) * t35966 - F::new(0.93943849568165663176e-4) * t35970 + F::new(0.29357452990051769742e-5) * t35973 - F::new(0.3757753982726626527e-3) * t35976 + F::new(0.19571635326701179828e-6) * t35979 + F::new(0.13298999885998441252e-6) * t35983 - F::new(0.93943849568165663176e-5) * t35986 - F::new(0.93943849568165663176e-5) * t35989 + F::new(0.55677388177399516375e-6) * t35992 + F::new(0.61871497612135212571e-7) * t35996 + F::new(0.65659062294300875668e-4) * t35999 - F::new(0.34798367610874697734e-7) * t36003 - F::new(0.65659062294300875668e-4) * t36006;
    t37599
}

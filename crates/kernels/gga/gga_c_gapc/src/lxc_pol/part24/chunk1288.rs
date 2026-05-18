//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1288/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1288<F: Float>(t35806: F, t35809: F, t35811: F, t35815: F, t35820: F, t35823: F, t35826: F, t35829: F, t35831: F, t35835: F, t35838: F, t35841: F, t35843: F) -> F {
    let t37539 = F::new(0.34798367610874697734e-6) * t35806 - F::new(0.93943849568165663176e-4) * t35809 + F::new(0.29161736636784757944e-3) * t35811 + F::new(0.76121429148788401293e-7) * t35815 + F::new(0.19571635326701179828e-6) * t35820 + F::new(0.93943849568165663176e-4) * t35823 + F::new(0.93943849568165663176e-4) * t35826 + F::new(0.46971924784082831588e-4) * t35829 - F::new(0.13678971311312682431e-5) * t35831 - F::new(0.45596571037708941436e-6) * t35835 + F::new(0.16703216453219854913e-4) * t35838 + F::new(0.58714905980103539484e-5) * t35841 + F::new(0.10578404480748474413e-3) * t35843;
    t37539
}

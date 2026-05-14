//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 795/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk795<F: Float>(t1063: F, t1064: F, t42086: F, t42821: F, t42822: F, t42823: F, t42824: F, t42826: F, t42828: F, t42829: F, t42832: F, t42835: F, t42838: F, t42841: F, t42844: F, t42845: F, t42847: F, t42850: F, t42852: F, t42857: F) -> (F,) {
    let t42861 = -t42821 - t42822 - t42823 + t42824 - t42826 + t42828 + 0.1138200265427045984e0 * t42829 + 0.1138200265427045984e0 * t42832 + 0.1138200265427045984e0 * t42835 + t42838 + t42841 - t42844 - t42845 + t42847 + t42850 - 0.12646669615856066489e-1 * t42852 + t42857 + 0.28455006635676149599e-1 * t1063 * t1064 * t42086;
    (t42861,)
}

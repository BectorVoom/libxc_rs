//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 614/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk614<F: Float>(t1701: F, t27494: F, t820: F, t2035: F, t6979: F, t811: F, t4061: F, t4093: F, t1472: F, t19039: F, t19101: F, t19107: F, t19132: F, t19135: F, t28540: F, t28544: F, t28595: F, t28655: F, t28663: F, t28677: F, t28680: F, t4094: F, t4099: F, t4104: F, t5265: F, t6976: F, t812: F, t821: F) -> (F,) {
    let t28684 = t1701 * t27494 * t820;
    let t28688 = t2035 * t6979 * t811;
    let t28692 = t2035 * t6979 * t820;
    let t28695 = t4061 * t4093;
    let t28716 = 0.24167761770734866966e0 * t28677 * t28655 - 0.24167761770734866966e0 * t28680 * t28663 - 0.12081826776807659559e1 * t1472 * t28684 - 0.10947790369858991997e1 * t19132 * t28688 + 0.54738951849294959987e0 * t19135 * t28692 - 0.45306850413028723348e0 * t28695 * t6976 - 0.45306850413028723348e0 * t4104 * t28595 - 0.22653425206514361674e0 * t4099 * t28540 - 0.24163653553615319118e1 * t4094 * t28544 + 0.12081826776807659559e1 * t4099 * t28684 + 0.21895580739717983994e1 * t19101 * t28688 - 0.10947790369858991997e1 * t19107 * t28692 - 0.54738951849294959988e0 * t19039 * t6979 * t812 + 0.27369475924647479994e0 * t5265 * t6979 * t821;
    (t28716,)
}

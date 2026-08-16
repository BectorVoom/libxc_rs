//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 668/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk668(t1701: f64, t27494: f64, t820: f64, t2035: f64, t6979: f64, t811: f64, t4061: f64, t4093: f64, t1472: f64, t19039: f64, t19101: f64, t19107: f64, t19132: f64, t19135: f64, t28540: f64, t28544: f64, t28595: f64, t28655: f64, t28663: f64, t28677: f64, t28680: f64, t4094: f64, t4099: f64, t4104: f64, t5265: f64, t6976: f64, t812: f64, t821: f64) -> f64 {
    let t28684 = t1701 * t27494 * t820;
    let t28688 = t2035 * t6979 * t811;
    let t28692 = t2035 * t6979 * t820;
    let t28695 = t4061 * t4093;
    let t28716 = 0.24167761770734866966e0_f64 * t28677 * t28655 - 0.24167761770734866966e0_f64 * t28680 * t28663 - 0.12081826776807659559e1_f64 * t1472 * t28684 - 0.10947790369858991997e1_f64 * t19132 * t28688 + 0.54738951849294959987e0_f64 * t19135 * t28692 - 0.45306850413028723348e0_f64 * t28695 * t6976 - 0.45306850413028723348e0_f64 * t4104 * t28595 - 0.22653425206514361674e0_f64 * t4099 * t28540 - 0.24163653553615319118e1_f64 * t4094 * t28544 + 0.12081826776807659559e1_f64 * t4099 * t28684 + 0.21895580739717983994e1_f64 * t19101 * t28688 - 0.10947790369858991997e1_f64 * t19107 * t28692 - 0.54738951849294959988e0_f64 * t19039 * t6979 * t812 + 0.27369475924647479994e0_f64 * t5265 * t6979 * t821;
    t28716
}
